import React from "react";
import { Route, Routes } from "react-router-dom";

import HomePage from "pages/Home/HomePage";
import NotFoundPage from "pages/NotFound/NotFoundPage";
import PageContainer from "./PageContainer";
import LoginPage from "pages/Login/LoginPage";
import TagsPage from "pages/Tags/TagsPage";
import SearchPage from "pages/Search/SearchPage";
import useAuthState from "hooks/useAuthState";
import { UserContext } from "util/UserContext";
import Loading from "components/Loading";
import NotLoggedInRedirect from "./NotLoggedInRedirect";

/**
 * Main component that handles global state fetching and rendering the page
 * container based on that
 */
const CoreContent: React.FC = () => {
  const authState = useAuthState();

  // TODO use suspense here once it hits React stable
  if (authState === "loading") {
    return <Loading size="8rem" />;
  }

  const isLoggedIn = authState;
  return (
    <UserContext.Provider value={{ isLoggedIn }}>
      <PageContainer showHeader={isLoggedIn}>
        {isLoggedIn ? (
          <Routes>
            <Route path="/" element={<HomePage />} />

            {/* <Route path="/search/:selectedUri?" element={<SearchPage />} /> */}
            <Route path="search" element={<SearchPage />}>
              <Route path=":selectedUri" element={<SearchPage />} />
            </Route>
            <Route path={"tags"} element={<TagsPage />}>
              <Route path=":selectedTag" element={<TagsPage />} />
            </Route>

            {/* Fallback route */}
            <Route path="*" element={<NotFoundPage />} />
          </Routes>
        ) : (
          // User is not logged in - redirect everything to the login page
          <Routes>
            <Route path="/" element={<LoginPage />} />

            <Route path="*" element={<NotLoggedInRedirect />} />
          </Routes>
        )}
      </PageContainer>
    </UserContext.Provider>
  );
};

export default CoreContent;
