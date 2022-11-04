xs <- list(1, 2, 5, 10)

for (x in xs) {
	data1 <- read.csv(sprintf("data_x%s.csv", x), header=TRUE)

	png(file = sprintf("ttfb_x%s.png", x))
	boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
	ylab = "Time to first byte", main = sprintf("Benchmark (%s)", x))
	dev.off()

	png(file = sprintf("ttc_x%s.png", x))
	boxplot(ttc ~ clients, data = data1, xlab = "# Clients",
	ylab = "Time to completion", main = sprintf("Benchmark (%s)", x))
	dev.off()
}

# head(subset(data1, data1$clients == 50))
