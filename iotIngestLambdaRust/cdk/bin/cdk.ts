#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { CdkIngestIotTegStack } from '../lib/cdk-stack';
import { getConfig } from '../lib/config';

const config = getConfig();

const app = new cdk.App();
new CdkIngestIotTegStack(app, 'CdkIngestIotTegStack', {
  env: {
    account: config.account,
    region: config.region,
  },
  config,
});