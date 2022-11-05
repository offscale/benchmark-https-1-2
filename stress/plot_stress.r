data <- read.csv("results/data_stress.csv", header=TRUE)


pdf(file = "results/stress.pdf")
par(mfrow = c(2, 1))
hist(data$ttfb * 1000,
     breaks='Scott',
     xlim=c(0,1000),
     col='blue',
     xlab="Time to first byte [ms]",
     main = "Stress test using 200 concurrent clients sustained over ~1h")

hist(data$ttc * 1000,
     breaks='Scott',
     xlim=c(0,2500),
     col='green',
     xlab="Time to completion [ms]",
     main = "")
dev.off()
