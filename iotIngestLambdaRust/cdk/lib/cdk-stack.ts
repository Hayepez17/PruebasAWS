import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as iot from 'aws-cdk-lib/aws-iot';
import * as sqs from 'aws-cdk-lib/aws-sqs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as lambdaEventSources from 'aws-cdk-lib/aws-lambda-event-sources';
import * as path from 'path';
import { ConfigProps } from './config';

type CdkIngestIotTegStackProps = cdk.StackProps & {
  config: Readonly<ConfigProps>;
}


export class CdkIngestIotTegStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: CdkIngestIotTegStackProps) {
    super(scope, id, props);

    const { config } = props;

    // Obtener la VPC por defecto
    const vpc = ec2.Vpc.fromLookup(this, 'DefaultVPC', {
      isDefault: true,
    });

    // Crear un grupo de seguridad
    const securityGroup = new ec2.SecurityGroup(this, 'ec2-sg-grafa-mysql-teg', {
      vpc,
      description: 'Allow SSH, HTTP, and custom TCP ports',
      allowAllOutbound: true, // Permitir todo el tráfico saliente
    });

    // Agregar reglas de entrada al grupo de seguridad
    securityGroup.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(22), 'Allow SSH access');
    securityGroup.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(80), 'Allow HTTP access');
    securityGroup.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(3000), 'Allow TCP access on port 3001');
    securityGroup.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(3306), 'Allow TCP access on port 3306');
    securityGroup.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(8686), 'Allow TCP access on port 8686');

    // //Script for install ec2-intance-connect
    // const userData = ec2.UserData.forLinux()
    // userData.addCommands(
    //   'apt-get update -y',
    //   'apt-get install -y git awscli ec2-instance-connect',
    //   'until git clone https://github.com/aws-quickstart/quickstart-linux-utilities.git; do echo "Retrying"; done',
    //   'cd /quickstart-linux-utilities',
    //   'source quickstart-cfn-tools.source',
    //   'qs_update-os || qs_err',
    //   'qs_bootstrap_pip || qs_err',
    //   'qs_aws-cfn-bootstrap || qs_err',
    //   'mkdir -p /opt/aws/bin',
    //   'ln -s /usr/local/bin/cfn-* /opt/aws/bin/',
    // )

    // Crear la instancia EC2
    const instance = new ec2.Instance(this, 'ec2-instance-grafana-mysql-teg-hy', {
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T2, ec2.InstanceSize.MICRO),
      keyPair: ec2.KeyPair.fromKeyPairName(this, 'KeyPair', 'ec2-grafana-teg-demo1'),
      machineImage: ec2.MachineImage.genericLinux({
        'us-east-1': 'ami-0f9de6e2d2f067fca', // Verifica la AMI en tu región para capa gratuita
      }),
      vpc,
      securityGroup,
    });

    // Crear una IP Elástica
    const eip = new ec2.CfnEIP(this, 'ec2-eip-teg-hy', {
      domain: 'vpc',
    });

    // Asociar la IP Elástica a la instancia EC2
    new ec2.CfnEIPAssociation(this, 'eip-ec2-teg-association', {
      eip: eip.attrPublicIp,
      instanceId: instance.instanceId,
    });

    // The code that defines your stack goes here
    // Create an S3 bucket
    const bucket = s3.Bucket.fromBucketName(this, 'ExistingBucket', config.bucketName); // Replace with your bucket name
    // If the bucket already exists in your AWS account, this will reference it instead of creating a new one.

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

    // Create a Lambda function with Rust runtime (no VPC, public internet access)
    const lambdaFunction = new lambda.Function(this, 'IoTIngestLambda', {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      code: lambda.Code.fromAsset(path.join(__dirname, "..", "..", "target/lambda/iot_ingest")), // Path to the compiled Rust binary
      handler: 'bootstrap',
      memorySize: 128,
      timeout: cdk.Duration.seconds(30),
      environment: {
      AWS_BUCKET_NAME: bucket.bucketName,
      QUEUE_URL: queue.queueUrl,
      DB_HOST_URL: eip.ref, // Usar la IP elástica asociada a la instancia EC2
      DB_PORT: config.dbPort,
      DB_USERNAME: config.dbUser,
      DB_PASSWORD: config.dbPassword,
      DB_NAME: config.dbName,
      },
      // No VPC or securityGroups, so Lambda has public internet access
    });

    // Grant the Lambda function write permissions to the S3 bucket
    bucket.grantPut(lambdaFunction);

    // Add the SQS queue as an event source for the Lambda function
    lambdaFunction.addEventSource(
      new lambdaEventSources.SqsEventSource(queue, {
        batchSize: 10, // Adjust the batch size as needed
        maxBatchingWindow: cdk.Duration.seconds(30), // Adjust the max batching window as needed
      })
    );

    // // Grant the Lambda function permissions to send messages to the SQS queue
    // queue.grantConsumeMessages(lambdaFunction);

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
