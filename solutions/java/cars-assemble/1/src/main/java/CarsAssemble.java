public class CarsAssemble {

    private final int CARS_PER_HOUR = 221;
    
    public double productionRatePerHour(int speed) {
        final int totalProduced = speed * CARS_PER_HOUR;

        double successRate;

        if (speed == 10)
            successRate = 0.77;
        else if (speed == 9)
            successRate = 0.80;
        else if (speed <= 8 && speed >= 5)
            successRate = 0.90;
        else if (speed <= 4 && speed >= 0)
            successRate = 1.0;
        else
            throw new IllegalArgumentException("Speed must be >= 0 and <= 10.");

        return totalProduced * successRate;
    }

    public int workingItemsPerMinute(int speed) {
        final double totalPerHour = productionRatePerHour(speed);
        return (int)(totalPerHour / 60.0);
    }
}
