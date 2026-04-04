public class LogLevels {
    
    public static String message(String logLine) {
        final String[] split = logLine.split("]:", 2);
        return split[1].trim();
    }

    public static String logLevel(String logLine) {
        final int start = logLine.indexOf("[") + 1;
        final int end = logLine.indexOf("]");
        final String level = logLine.substring(start, end);
        return level.toLowerCase();
    }

    public static String reformat(String logLine) {
        return message(logLine) + " (" + logLevel(logLine) + ")";
    }
}
