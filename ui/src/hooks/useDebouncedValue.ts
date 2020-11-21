import { useEffect, useState, useCallback } from "react";
import { debounce } from "lodash-es";

/**
 * Debounces an input value, so that any changes to it aren't reflected in the
 * output until after a delay. Any changes that occur during the delay will
 * reset the delay. The new output value won't appear until after the until
 * value hasn't changed for the defined wait time, at which point the most
 * recent input value will become the output.
 * @param value The current value
 * @param wait The amount of milliseconds to wait after a value change before updating the output
 * @return The debounced value
 */
const useDebouncedValue = <T>(value: T, wait: number): T => {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const debouncedSetter = useCallback(
    debounce((newValue: T) => {
      setDebouncedValue(newValue);
    }, wait),
    [wait]
  );

  useEffect(() => {
    debouncedSetter(value);
  }, [value, debouncedSetter]);

  return debouncedValue;
};

export default useDebouncedValue;
