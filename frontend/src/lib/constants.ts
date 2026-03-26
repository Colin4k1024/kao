export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || '/api'

export const TOKEN_KEY = 'token'

export const USER_INFO_KEY = 'userInfo'

export const PAGE_SIZE_OPTIONS = [10, 20, 50, 100]

export const DEFAULT_PAGE_SIZE = 20

export const DATE_FORMAT = 'yyyy-MM-dd'

export const DATETIME_FORMAT = 'yyyy-MM-dd HH:mm:ss'

export const TIME_FORMAT = 'HH:mm:ss'

export const SYSTEM_ROLES = {
  ADMIN: 'admin',
  USER: 'user',
  GUEST: 'guest',
} as const

export const USER_STATUS = {
  ACTIVE: 1,
  INACTIVE: 0,
  LOCKED: 2,
} as const

export const MENU_TYPES = {
  CATALOG: 'M',
  MENU: 'C',
  BUTTON: 'B',
} as const

export const HTTP_STATUS = {
  OK: 200,
  CREATED: 201,
  NO_CONTENT: 204,
  BAD_REQUEST: 400,
  UNAUTHORIZED: 401,
  FORBIDDEN: 403,
  NOT_FOUND: 404,
  INTERNAL_SERVER_ERROR: 500,
} as const

export const RESPONSE_CODE = {
  SUCCESS: 200,
  ERROR: 500,
  UNAUTHORIZED: 401,
  FORBIDDEN: 403,
} as const
