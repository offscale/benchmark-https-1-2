data1 <- read.csv("data_x1.csv", header=TRUE)

png(file = "ttfb_x1.png")
boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to first byte", main = "Benchmark (x1)")
dev.off()

png(file = "ttc_x1.png")
boxplot(ttc ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to completion", main = "Benchmark (x1)")
dev.off()





data1 <- read.csv("data_x2.csv", header=TRUE)

png(file = "ttfb_x2.png")
boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to first byte", main = "Benchmark (x2)")
dev.off()

png(file = "ttc_x2.png")
boxplot(ttc ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to completion", main = "Benchmark (x2)")
dev.off()




data1 <- read.csv("data_x5.csv", header=TRUE)

png(file = "ttfb_x5.png")
boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to first byte", main = "Benchmark (x5)")
dev.off()

png(file = "ttc_x5.png")
boxplot(ttc ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to completion", main = "Benchmark (x5)")
dev.off()




data1 <- read.csv("data_x10.csv", header=TRUE)

png(file = "ttfb_x10.png")
boxplot(ttfb ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to first byte", main = "Benchmark (x10)")
dev.off()

png(file = "ttc_x10.png")
boxplot(ttc ~ clients, data = data1, xlab = "# Clients",
   ylab = "Time to completion", main = "Benchmark (x10)")
dev.off()


