UPLOAD_TARGET=lambda-$(git rev-parse --short HEAD).zip
aws s3 cp $UPLOAD_TARGET s3://jodavaho-spacetime/runtime/$UPLOAD_TARGET
aws s3 sync data/ s3://jodavaho-spacetime/data/
aws lambda update-function-code --function-name moonapi --s3-bucket jodavaho-spacetime --s3-key runtime/$UPLOAD_TARGET
