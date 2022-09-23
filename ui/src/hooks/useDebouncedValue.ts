import { useEffect, useState } from "react";

/**
 * Debounces an input value, so that any changes to it aren't reflected in the
 * output until after a delay. Any changes that occur during the delay will
 * reset the delay. The new output value won't appear until after the until
 * value hasn't changed for the defined wait time, at which point the most
 * recent input value will become the output.
 * @param value The current value
 * @param waitMs The amount of milliseconds to wait after a value change before updating the output
 * @return The debounced value
 */
function useDebouncedValue<T>(value: T, waitMs: number): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const timeoutId = setTimeout(() => setDebouncedValue(value), waitMs);
    return () => clearTimeout(timeoutId);
  }, [waitMs, value]);

  return debouncedValue;
}

export default useDebouncedValue;
