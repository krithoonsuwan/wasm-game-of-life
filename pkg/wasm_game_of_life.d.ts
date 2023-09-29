/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Cell {
  Dead = 0,
  Alive = 1,
}
/**
*/
export class Universe {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @returns {Universe}
*/
  static new(width: number, height: number): Universe;
/**
* Set the width of the universe.
*
* Resets all cells to the dead state.
* @param {number} width
*/
  set_width(width: number): void;
/**
* @returns {number}
*/
  width(): number;
/**
* Set the height of the universe.
*
* Resets all cells to the dead state.
* @param {number} height
*/
  set_height(height: number): void;
/**
* @returns {number}
*/
  height(): number;
/**
* Set cells to be alive in a universe by passing the row and column
* of each cell as an array.
* @param {Uint32Array} rows
* @param {Uint32Array} cols
*/
  set_cells(rows: Uint32Array, cols: Uint32Array): void;
/**
* @returns {number}
*/
  cells(): number;
/**
* @param {number} row
* @param {number} column
*/
  toggle_cell(row: number, column: number): void;
/**
* @returns {string}
*/
  render(): string;
/**
*/
  tick(): void;
}
