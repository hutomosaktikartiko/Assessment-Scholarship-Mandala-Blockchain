function printLeftTriangle(n) {
  /**
    * Prints a left-aligned triangle with n rows.
    *
    * @param {number} n - The number of rows in the triangle
    * @returns {void}
  */
  for (let i = 1; i <= n; i++) {
    console.log('*'.repeat(i));
  }
}

function printCenteredTriangle(n) {
  /**
    * Prints a centered triangle with n rows.
    *
    * @param {number} n - The number of rows in the triangle
    * @returns {void}
  */
  for (let i = 1; i <= n; i++) {
    let spaces = ' '.repeat(n - i);
    let stars = '*'.repeat(2 * i - 1);
    console.log(spaces + stars);
  }
}

// Example usage:
function main() {
  // You can replace this with your own input method
  const n = 5;

  console.log("Left-ligned triangle:");
  printLeftTriangle(n);

  console.log("\nCentered triangle:");
  printCenteredTriangle(n);
}

main();