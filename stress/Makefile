help:
	echo "help"

plot-all: results/run.pdf results/stress.pdf results/run_detailed.pdf

benchmark:
	sh run.sh

stress200:
	# ~ 1 second per download. So let's run this for 4000 seconds :)
	echo "Running a stress test with continuous 200 clients"
	cargo run --release -- -N 800000 -C 200 ${URL} > results/stress_200_x4000.json

results/run.pdf: plot.r results/data.csv Makefile
	Rscript plot.r onepage

results/run_detailed.pdf: plot.r results/data.csv Makefile
	Rscript plot.r multipage

results/stress.pdf: plot_stress.r results/data_stress.csv Makefile
	Rscript plot_stress.r

results/data_stress.csv: results/stress_200_x4000.json gendata.rb
	ruby gendata.rb results/stress_*.json > results/data_stress.csv

results/data.csv: gendata.rb
	ruby gendata.rb results/run_*.json > results/data.csv
