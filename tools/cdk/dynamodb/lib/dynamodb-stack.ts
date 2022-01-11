import { Stack, StackProps, RemovalPolicy } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as s3n from 'aws-cdk-lib/aws-s3-notifications';
import * as eventsources from 'aws-cdk-lib/aws-lambda-event-sources';
import * as path from 'path';


export class DynamodbStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const bucket = new s3.Bucket(this, "rustbucket", {
        versioned: false,
        removalPolicy: RemovalPolicy.DESTROY,
        enforceSSL: true,
    });

    let docker_dir = path.join(__dirname, '../../../sls/dynamodb');
    
    let fn = new lambda.DockerImageFunction(this, 'rust-dynamodb', {
        description: 
            'Rust on lambda with dynamodb',
        code: lambda.DockerImageCode.fromImageAsset( docker_dir ), 
        architecture: lambda.Architecture.ARM_64,
        environment: {
            RUST_BACKTRACE: '1',
        } ,

        logRetention: RetentionDays.ONE_WEEK,
    });
    bucket.addEventNotification(s3.EventType.OBJECT_CREATED, new s3n.LambdaDestination(fn));
  }
}