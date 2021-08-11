#!/bin/bash

if [ -z $2 ];then
	echo "用法错误。需要输入2个参数："
	echo "	第一个参数：开始区块序号"
	echo "	第二个参数：提取区块数量"
	echo "请重新执行。"
	exit
fi

i=$1
block_num=`expr $1 + $2`
while [ $i -lt $block_num ]
do
	hex_id=`printf "0x%x" $i`
	((i++)) 
	echo "提取区块: ${blockId} 数据..."
	curl -X POST --data '{"jsonrpc":"2.0","method":"getBlockByNumber","params":[1,"${hex_id}",true],"id":1}' http://127.0.0.1:8545 > ${hex_id}.json
done
exit
