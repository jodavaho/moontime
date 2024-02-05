cargo lambda build --release -j8
zip lambda.zip -j ./target/lambda/moontime/bootstrap 
aws s3 cp lambda.zip s3://jodavaho-spacetime/runtime/lambda.zip
aws lambda update-function-code --function-name moonapi --s3-bucket jodavaho-spacetime --s3-key runtime/lambda.zip
