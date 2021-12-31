#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { MlMechanicStack } from '../lib/ml-mechanic-stack';

const app = new cdk.App();
new MlMechanicStack(app, 'MlMechanicStack', {
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
});
