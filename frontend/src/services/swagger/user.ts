// @ts-ignore
/* eslint-disable */
import { api } from '@/lib/api';

/** Create user This can only be done by the logged in user. POST /user */
export async function createUser(body: API.User, options?: { [key: string]: any }) {
  return api.post<any>('/user', body, options);
}

/** Get user by user name GET /user/${param0} */
export async function getUserByName(
  // 叠加生成的Param类型 (非body参数swagger默认没有生成对象)
  params: API.getUserByNameParams,
  options?: { [key: string]: any },
) {
  const { username: param0, ...queryParams } = params;
  return api.get<API.User>(`/user/${param0}`, { params: { ...queryParams }, ...options });
}

/** Updated user This can only be done by the logged in user. PUT /user/${param0} */
export async function updateUser(
  // 叠加生成的Param类型 (非body参数swagger默认没有生成对象)
  params: API.updateUserParams,
  body: API.User,
  options?: { [key: string]: any },
) {
  const { username: param0, ...queryParams } = params;
  return api.put<any>(`/user/${param0}`, body, { params: { ...queryParams }, ...options });
}

/** Delete user This can only be done by the logged in user. DELETE /user/${param0} */
export async function deleteUser(
  // 叠加生成的Param类型 (非body参数swagger默认没有生成对象)
  params: API.deleteUserParams,
  options?: { [key: string]: any },
) {
  const { username: param0, ...queryParams } = params;
  return api.delete<any>(`/user/${param0}`, { params: { ...queryParams }, ...options });
}

/** Creates list of users with given input array POST /user/createWithArray */
export async function createUsersWithArrayInput(
  body: API.User[],
  options?: { [key: string]: any },
) {
  return api.post<any>('/user/createWithArray', body, options);
}

/** Creates list of users with given input array POST /user/createWithList */
export async function createUsersWithListInput(body: API.User[], options?: { [key: string]: any }) {
  return api.post<any>('/user/createWithList', body, options);
}

/** Logs user into the system GET /user/login */
export async function loginUser(
  // 叠加生成的Param类型 (非body参数swagger默认没有生成对象)
  params: API.loginUserParams,
  options?: { [key: string]: any },
) {
  return api.get<string>('/user/login', { params, ...options });
}

/** Logs out current logged in user session GET /user/logout */
export async function logoutUser(options?: { [key: string]: any }) {
  return api.get<any>('/user/logout', options);
}
