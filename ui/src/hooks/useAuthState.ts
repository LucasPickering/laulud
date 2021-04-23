import { useEffect, useState } from "react";

/**
 * Check if the user is logged in. Each instance of this hook will make a single
 * HTTP request.
 * @returns true if user is authed, false if not, 'loading' if the request is
 *  still loading
 */
function useAuthState(): boolean | "loading" {
  const [authState, setAuthState] = useState<boolean | "loading">("loading");

  // This API returns a 200 if authed, 401 if not
  useEffect(() => {
    fetch("/api/auth-check")
      .then((response) => setAuthState(response.ok))
      .catch(() => setAuthState(false));
  }, []);

  return authState;
}

export default useAuthState;
