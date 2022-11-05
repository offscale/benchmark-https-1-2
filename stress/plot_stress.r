data <- read.csv("results/data_stress.csv", header=TRUE)


pdf(file = "results/stress.pdf")
par(mfrow = c(2, 1))
hist(data$ttfb, breaks=100, xlim=c(0,2), col='black',
     xlab="Time to first byte [s]", main = "Stress test using 200 concurrent clients sustained over ~1h")

hist(data$ttc, breaks=100, xlim=c(0,2), col='blue',
     xlab="Time to completion [s]", main = "")
dev.off()
