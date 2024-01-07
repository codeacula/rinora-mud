import { App, Ref, ref } from "vue";

export interface Authentication {
  isAuthenticated: Ref<boolean>;
  updateIsAuthenticated: (isAuthFlag: boolean) => void;
}

export default function useAuthentication(app: App) {
  const isAuthenticated = ref(false);
  const updateIsAuthenticated = (isAuthFlag: boolean) => {
    isAuthenticated.value = isAuthFlag;
  };

  app.provide<Authentication>("authentication", {
    isAuthenticated,
    updateIsAuthenticated,
  });
}
