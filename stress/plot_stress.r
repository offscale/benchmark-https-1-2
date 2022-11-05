data <- read.csv("data_stress.csv", header=TRUE)


png(file = "stress.png")
par(mfrow = c(2, 1))
hist(data$ttfb, breaks=100, xlim=c(0,2), col='black',
     xlab="Time to first byte [s]", main = "Stress test using 200 concurrent clients sustained over ~1h")

hist(data$ttc, breaks=100, xlim=c(0,2), col='blue',
     xlab="Time to completion [s]", main = "")

#boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
#boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
#ylab = "Time to first byte", main = "Benchmark Stress (200 clients / 4000 reqs/per clients)")
dev.off()
