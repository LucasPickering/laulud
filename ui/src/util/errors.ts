export class UnknownItemTypeError extends Error {
  constructor(private readonly itemType: string) {
    super();
  }

  public toString(): string {
    return `Unknown item type: ${this.itemType}`;
  }
}
