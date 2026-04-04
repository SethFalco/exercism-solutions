// @ts-check

/** Types of vehicles that require a license. */
const VEHICLES_REQ_LICENSE = [
  'car',
  'truck'
];

/**
 * Determines whether or not you need a licence to operate a certain kind of vehicle.
 *
 * @param {string} type
 * @returns {boolean} whether a license is required
 */
export function needsLicense(type) {
  return VEHICLES_REQ_LICENSE.includes(type);
}

/**
 * Helps choosing between two options by recommending the one that
 * comes first in dictionary order.
 *
 * @param {string} option1
 * @param {string} option2
 * @returns {string} a sentence of advice which option to choose
 */
export function chooseVehicle(option1, option2) {
  const rec = (option1 < option2) ? option1 : option2;
  return rec + ' is clearly the better choice.';
}

/**
 * Calculates an estimate for the price of a used vehicle in the dealership
 * based on the original price and the age of the vehicle.
 *
 * @param {number} originalPrice
 * @param {number} age
 * @returns {number} expected resell price in the dealership
 */
export function calculateResellPrice(originalPrice, age) {
  if (age > 10) {
    return originalPrice * 0.5;
  }

  if (age >= 3) {
    return originalPrice * 0.7;
  }

  return originalPrice * 0.8;
}
