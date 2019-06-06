# Select

Select component tries to replicate and extend the functionality of the native browser `select` element, styling it as specified in the design guideline.

## Behaviour
- Accepts either `options`, or groups of `options` (groups are separated visually in the options dropdown).
- Upon selecting an option, calls the `onSelect` handler passing to it the value of the `value` field of the selected option object.
- Allows navigation using keyboard (`enter` opens the dropdown with options, `up` and `down` arrow keys cycle between the options).
- If the list of options is too tall to fit the space between the `select` element and the bottom border of the window, its height is adjusted to fit the allotted space. In such a case, the options panel shows areas (marked with up and down arrows) that initiate scrolling up or down when moused over. Taking the cursor away from these areas stops scrolling. Cycling between the options using arrow keys should also work; and each change of the option should scroll the newly highlighted option into view.
