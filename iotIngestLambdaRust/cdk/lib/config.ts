import * as dotenv from 'dotenv';
import path = require('path');

dotenv.config({ path: path.resolve(__dirname, '../../prod.env') });

export type ConfigProps = {
    dbHostUrl: string;
    dbPort: string;
    dbUser: string;
    dbPassword: string;
    dbName: string;
    bucketName: string;
    region: string;
    account: string;
}

export const getConfig = (): ConfigProps => {

    return {
        region: process.env.CDK_DEFAULT_REGION || 'us-east-1',
        account: process.env.CDK_DEFAULT_ACCOUNT || '123456789012',
        // Database configuration
        dbHostUrl: process.env.DB_HOST_URL || 'mysql',
        dbPort: process.env.DB_PORT || '3306', // Default MySQL port
        dbUser: process.env.DB_USER || 'admin',
        dbPassword: process.env.DB_PASSWORD || 'admin',
        dbName: process.env.DB_NAME || 'mydb',
        // S3 bucket name
        bucketName: process.env.AWS_BUCKET_NAME || 'my-s3-bucket',
    }
}