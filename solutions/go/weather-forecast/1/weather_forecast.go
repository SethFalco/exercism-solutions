// Package weather: For checking the weather forecast.
package weather

// CurrentCondition: The last condition that Forecase was called with.
var CurrentCondition string

// CurrentLocation: The last location that Forecast was called with.
var CurrentLocation string

// Forecast: Returns the a string representing of the weather forecast.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
