URL?="https://cdn.playable.video/samples.playable.video/v:4779304402550784/0/4779304402550784/16:9/-/3.mp4?u=1997dab88b945bf93867173a7f4cfe558a04ec4deba0591ce649552b0b473134"

benchmark:
	sh run.sh

stress200:
	# ~ 1 second per download. So let's run this for 4000 seconds :)
	echo "Running a stress test with continuous 200 clients"
	cargo run --release -- -N 800000 -C 200 ${URL} > stress_200_x4000.json

plot:
	ruby gendata.rb > data.csv
	R --no-save < plot.r

_benchmark:
	cargo run --release -- -N 1 -C 1 ${URL} > 1_1.json


