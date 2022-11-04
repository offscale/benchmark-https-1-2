URL="https://cdn.playable.video/samples.playable.video/v:4779304402550784/0/4779304402550784/16:9/-/3.mp4?u=1997dab88b945bf93867173a7f4cfe558a04ec4deba0591ce649552b0b473134"

cargo build --release || exit 1

for reqs_per_client in 1 2 5 10; do
	for clients in 500 1000; do
		./target/release/stress \
			-N `expr ${clients} \* ${reqs_per_client}` \
			-C ${clients} \
			> ${clients}_x${reqs_per_client}.json ${URL} || exit 1
	done
done
