public class Lasagna {
    
    public int expectedMinutesInOven() {
        return 40;
    }

    public int remainingMinutesInOven(int actualMinutes) {
        return expectedMinutesInOven() - actualMinutes;
    }

    public int preparationTimeInMinutes(int layerCount) {
        return layerCount * 2;
    }

    public int totalTimeInMinutes(int layerCount, int actualMinutes) {
        return preparationTimeInMinutes(layerCount) + actualMinutes;
    }
}
