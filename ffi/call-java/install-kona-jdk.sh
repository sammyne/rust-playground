#!/bin/bash

set -e

remote=https://github.com/Tencent/TencentKona-8/releases/download/8.0.8-GA/TencentKona8.0.8.b1_jdk_linux-x86_64_8u312.tar.gz
archive=$(basename $remote)
install_dir=/opt

curl -LO $remote

folder=$(tar -tf $archive | awk -F'/' '{print $1}' | head -1)

rm -rf $install_dir/$folder
tar -xvf $archive -C $install_dir

echo ""
echo "export JAVA_HOME=$install_dir/$folder" >> ~/.bashrc
echo 'export PATH=${JAVA_HOME}/bin:$PATH' >> ~/.bashrc
echo 'export CLASSPATH=.:${JAVA_HOME}/lib' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=${JAVA_HOME}/jre/lib/amd64/server:$LD_LIBRARY_PATH' >> ~/.bashrc
echo ""
#cat ~/.bashrc

source ~/.bashrc

java -version

rm -rf $archive
