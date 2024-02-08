#https://5m1ktm7nie.execute-api.us-east-1.amazonaws.com/
echo "Testing API Gateway"
echo "=========================="
curl -Li https://5m1ktm7nie.execute-api.us-east-1.amazonaws.com/
echo ""
echo ""
echo "Testing API Gateway/wtf"
echo "=========================="
curl -Li https://5m1ktm7nie.execute-api.us-east-1.amazonaws.com/wtf
echo ""
echo ""
echo "Testing API Gateway/nonproxy"
echo "=========================="
curl -Li https://5m1ktm7nie.execute-api.us-east-1.amazonaws.com/nonproxy
echo ""
echo ""
echo "Testing API Gateway/Prod"
echo "=========================="
curl -Li https://5m1ktm7nie.execute-api.us-east-1.amazonaws.com/Prod/
echo ""
echo ""
echo "Testing API Gateway/Prod/wtf"
echo "=========================="
curl -Li https://5m1ktm7nie.execute-api.us-east-1.amazonaws.com/Prod/wtf
echo ""
echo ""
echo "Testing API Gateway/Prod/nonproxy"
echo "=========================="
curl -Li https://5m1ktm7nie.execute-api.us-east-1.amazonaws.com/Prod/nonproxy
echo ""
echo ""
echo "Testing Lambda direct"
echo "=========================="
curl -Li https://qg42vnjn6oj7vmq5hch6u5xdyy0rssvk.lambda-url.us-east-1.on.aws/
echo ""
echo ""
echo "Testing Lambda direct/wtf"
echo "=========================="
curl -Li https://qg42vnjn6oj7vmq5hch6u5xdyy0rssvk.lambda-url.us-east-1.on.aws/wtf
echo ""
echo ""
echo "custom url"
echo "=========================="
curl -Li https://api.jodavaho.io/s/
echo ""
echo ""
echo "custom url/wtf"
echo "=========================="
curl -Li https://api.jodavaho.io/s/wtf
echo ""
echo ""
echo "custom url/nonproxy"
echo "=========================="
curl -Li https://api.jodavaho.io/s/nonproxy
echo ""
