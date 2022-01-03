import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import * as path from 'path';


export class MlMechanicStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    let docker_dir = path.join(__dirname, '../tools/spanner');
    
    new lambda.DockerImageFunction(this, 'rust-hello', {
        description: 
            'Rust on lambda, using arm!',
        code: lambda.DockerImageCode.fromImageAsset( docker_dir ), 
        architecture: lambda.Architecture.ARM_64,
        environment: {
            RUST_BACKTRACE: '1',
        } ,
        logRetention: RetentionDays.ONE_WEEK,
    });
  }
}
