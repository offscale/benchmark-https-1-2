URL?="https://cdn.playable.video/samples.playable.video/v:4779304402550784/0/4779304402550784/16:9/-/3.mp4?u=1997dab88b945bf93867173a7f4cfe558a04ec4deba0591ce649552b0b473134"

benchmark:
	sh run.sh

_benchmark:
	cargo run --release -- -N 1 -C 1 ${URL} > 1_1.json

plot:
	ruby gendata.rb
	R --no-save < plot.r
