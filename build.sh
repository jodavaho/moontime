set -e
CSPICE_DIR=/home/hook/ws/cspice/cspice cargo build --release -j8
echo "Build succeeded!"
echo "Making lambda ... "
sleep 1
CSPICE_DIR=/home/hook/ws/cspice/cspice cargo lambda build --release -j8
ls -l -h ./target/lambda/moontime/bootstrap
rm lambda.zip
zip lambda.zip -j ./target/lambda/moontime/bootstrap
zip lambda.zip  data/latest_leapseconds.tls
zip lambda.zip data/de440s.bsp
zip lambda.zip data/moon_pa_de440_200625.bpc
zip lambda.zip data/moon_de440_200625.tf
zip lambda.zip data/pck00010.tpc
ls -lh lambda.zip
unzip -l lambda.zip
cp lambda.zip lambda-$(git rev-parse --short HEAD).zip
