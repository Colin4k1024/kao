import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface AuthState {
  token: string | null
  userInfo: any | null
  isAuthenticated: boolean
  login: (token: string, userInfo: any) => void
  logout: () => void
  updateUserInfo: (userInfo: any) => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      userInfo: null,
      isAuthenticated: false,
      login: (token, userInfo) => set({ 
        token, 
        userInfo, 
        isAuthenticated: true 
      }),
      logout: () => set({ 
        token: null, 
        userInfo: null, 
        isAuthenticated: false 
      }),
      updateUserInfo: (userInfo) => set({ userInfo }),
    }),
    {
      name: 'auth-storage',
    }
  )
)
