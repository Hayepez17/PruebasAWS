import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as iot from 'aws-cdk-lib/aws-iot';
import * as sqs from 'aws-cdk-lib/aws-sqs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as lambdaEventSources from 'aws-cdk-lib/aws-lambda-event-sources';
// import * as sqs from 'aws-cdk-lib/aws-sqs';


const securityGroupId = process.env.SECURITY_GROUP_ID || 'sg-xxxxxxxx'; // Replace 'sg-xxxxxxxx' with a default value or leave it empty
const dbHostUrl = process.env.DB_HOST_URL || 'your-db-host-url'; // Replace with your actual DB host URL
const dbPort = process.env.DB_PORT || '3306'; // Replace with your actual DB port

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here
    // Create an S3 bucket
    const bucket = new s3.Bucket(this, 'IoTIngestBucket', {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    // Create an SQS queue
    const queue = new sqs.Queue(this, 'InlineDataQueue', {
      visibilityTimeout: cdk.Duration.seconds(300),
    });
        // // Create an SQS queue with dead-letter queue for batch error handling
        // const deadLetterQueue = new sqs.Queue(this, 'DeadLetterQueue', {
        //   retentionPeriod: cdk.Duration.days(14),
        // });
    
        // const queue = new sqs.Queue(this, 'InlineDataQueue', {
        //   visibilityTimeout: cdk.Duration.seconds(300),
        //   deadLetterQueue: {
        //   maxReceiveCount: 5, // Number of times a message can be received before moving to the dead-letter queue
        //   queue: deadLetterQueue,
        //   },
        // });
    
    // Use an existing security group
    const vpc = ec2.Vpc.fromLookup(this, 'DefaultVpc', { isDefault: true });
    
    const securityGroup = ec2.SecurityGroup.fromSecurityGroupId(this, 'ExistingSecurityGroup', securityGroupId, {
      mutable: true, // Set to true if you want to modify the security group
    });

    // Create a Lambda function with Rust runtime
    const lambdaFunction = new lambda.Function(this, 'IoTIngestLambda', {
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset('../target/lambda/iotIngestLambdaRust'), // Path to the compiled Rust binary
      handler: 'bootstrap',
      memorySize: 128,
      timeout: cdk.Duration.seconds(30),
      environment: {
      BUCKET_NAME: bucket.bucketName,
      DB_HOST_URL: dbHostUrl,
      DB_PORT: dbPort,
      },
      securityGroups: [securityGroup],
      vpc,
      vpcSubnets: {
      subnetType: ec2.SubnetType.PRIVATE_WITH_EGRESS, // Ensure the Lambda function is placed in private subnets
      },
    });

    // Grant the Lambda function write permissions to the S3 bucket
    bucket.grantWrite(lambdaFunction);

    // Add the SQS queue as an event source for the Lambda function
    lambdaFunction.addEventSource(new lambdaEventSources.SqsEventSource(queue));

    // Grant the Lambda function permissions to send messages to the SQS queue
    queue.grantConsumeMessages(lambdaFunction);

    // Create an IoT Core rule
    new iot.CfnTopicRule(this, 'IoTCoreRule', {
      topicRulePayload: {
      sql: "SELECT *, topic(2) as mac FROM 'M5Stack/+/test'",
      actions: [
        {
        sqs: {
          queueUrl: queue.queueUrl,
          roleArn: new iam.Role(this, 'IoTSqsRole', {
          assumedBy: new iam.ServicePrincipal('iot.amazonaws.com'),
          inlinePolicies: {
            sqsPublishPolicy: new iam.PolicyDocument({
            statements: [
              new iam.PolicyStatement({
              actions: ['sqs:SendMessage'],
              resources: [queue.queueArn],
              }),
            ],
            }),
          },
          }).roleArn,
          useBase64: false,
        },
        },
      ],
      },
    });
    // example resource
    // const queue = new sqs.Queue(this, 'CdkQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
