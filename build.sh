CSPICE_DIR=/home/hook/ws/cspice/cspice cargo lambda build --release -j8
ls -l -h ./target/lambda/moontime/bootstrap
zip lambda.zip -j ./target/lambda/moontime/bootstrap 
zip lambda.zip \
  data/latest_leapseconds.tls \
  data/de440.bsp \
  data/moon_pa_de440_200625.bpc \
  data/moon_de440_220930.tf \
  data/pck00010.tpc 
ls -lh lambda.zip
cp lambda.zip lambda-$(git rev-parse --short HEAD).zip
