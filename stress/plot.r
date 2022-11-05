args<-commandArgs(TRUE)

xs <- list(1, 2, 5, 10)

data <- read.csv("results/data.csv", header=TRUE)


if (args == "onepage") {
	pdf(file = "results/run.pdf")
	par(mfrow = c(4, 2))
} else {
	pdf(file = "results/run_detailed.pdf")

}

for (x in xs) {
	data1 = subset(data, data$x == x)
	data1 = subset(data1, data$clients <= 200)

	boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
	ylab = "Time to first byte [s]", main = sprintf("Reqs/client (%s)", x))

	boxplot(ttc ~ clients, data = data1, xlab = "# Clients",
	ylab = "Time to completion [s]", main = sprintf("Reqs/client (%s)", x))
}
dev.off()
