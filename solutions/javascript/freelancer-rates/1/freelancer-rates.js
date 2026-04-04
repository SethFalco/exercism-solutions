// @ts-check

const WORK_HOURS_PER_DAY = 8;
const WORK_DAYS_PER_MONTH = 22;

/**
 * The day rate, given a rate per hour
 *
 * @param {number} ratePerHour
 * @returns {number} the rate per day
 */
export function dayRate(ratePerHour) {
  return WORK_HOURS_PER_DAY * ratePerHour;
}

/**
 * Calculates the number of days in a budget, rounded down
 *
 * @param {number} budget: the total budget
 * @param {number} ratePerHour: the rate per hour
 * @returns {number} the number of days
 */
export function daysInBudget(budget, ratePerHour) {
  return Math.floor(budget / dayRate(ratePerHour));
}

/**
 * Calculates the discounted rate for large projects, rounded up
 *
 * @param {number} ratePerHour
 * @param {number} numDays: number of days the project spans
 * @param {number} discount: for example 20% written as 0.2
 * @returns {number} the rounded up discounted rate
 */
export function priceWithMonthlyDiscount(ratePerHour, numDays, discount) {
  const ratePerDay = dayRate(ratePerHour);
  const ratePerMonth = WORK_DAYS_PER_MONTH * ratePerDay;

  const months = Math.floor(numDays / WORK_DAYS_PER_MONTH);
  const remainingDays = numDays - (months * WORK_DAYS_PER_MONTH);

  const discountedBill = (months * ratePerMonth * (1 - discount));
  const remainingBill = remainingDays * ratePerDay;

  return Math.ceil(discountedBill + remainingBill);
}
