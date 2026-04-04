// @ts-check

/**
 * The fast attack is available when the knight is sleeping
 *
 * @param {boolean} knightAwake
 *
 * @return {boolean} Whether or not you can execute a fast attack.
 */
export function canExecuteFastAttack(knightAwake) {
  return !knightAwake;
}

/**
 * A useful spy captures information, which they can't do if everyone's asleep.
 *
 * @param {boolean} knightAwake
 * @param {boolean} archerAwake
 * @param {boolean} prisonerAwake
 *
 * @returns {boolean} Whether or not you can spy on someone.
 */
export function canSpy(knightAwake, archerAwake, prisonerAwake) {
  return knightAwake || archerAwake || prisonerAwake;
}

/**
 * You'll get caught by the archer if you signal while they're awake.
 *
 * @param {boolean} archerAwake
 * @param {boolean} prisonerAwake
 *
 * @returns {boolean} Whether or not you can send a signal to the prisoner.
 */
export function canSignalPrisoner(archerAwake, prisonerAwake) {
  return !archerAwake && prisonerAwake;
}

/**
 * The final stage in the plan: freeing Annalyn's best friend.
 *
 * @param {boolean} knightAwake
 * @param {boolean} archerAwake
 * @param {boolean} prisonerAwake
 * @param {boolean} petDogPresent
 *
 * @returns {boolean} Whether or not you can free Annalyn's friend.
 */
export function canFreePrisoner(knightAwake, archerAwake, prisonerAwake, petDogPresent) {
  if (petDogPresent) {
    return !archerAwake;
  }

  return prisonerAwake && !knightAwake && !archerAwake;
}
