export interface Dictionary {
  id: number
  name: string
  code: string
  description?: string
  status: number
  createdAt: string
  updatedAt: string
}

export interface DictionaryItem {
  id: number
  dictId: number
  label: string
  value: string
  sort: number
  status: number
  remark?: string
  createdAt: string
  updatedAt: string
}

export interface Parameter {
  id: number
  name: string
  key: string
  value: string
  type: 'string' | 'number' | 'boolean'
  description?: string
  status: number
  createdAt: string
  updatedAt: string
}

export interface Notice {
  id: number
  title: string
  content: string
  type: number
  status: number
  publishAt?: string
  author?: string
  createdAt: string
  updatedAt: string
}

export interface Job {
  id: number
  name: string
  jobGroup: string
  jobType: string
  cronExpression?: string
  interval?: number
  status: number
  concurrent: boolean
  misfirePolicy: string
  description?: string
  className?: string
  methodName?: string
  params?: string
  createdAt: string
  updatedAt: string
}

export interface JobLog {
  id: number
  jobId: number
  jobName: string
  status: number
  startTime: string
  endTime?: string
  duration?: number
  error?: string
  createdAt: string
}

export interface DictionaryQueryParams {
  name?: string
  code?: string
  status?: number
}

export interface DictionaryItemQueryParams {
  dictId?: number
  label?: string
  value?: string
  status?: number
}

export interface ParameterQueryParams {
  name?: string
  key?: string
  status?: number
}

export interface NoticeQueryParams {
  title?: string
  type?: number
  status?: number
}

export interface JobQueryParams {
  name?: string
  jobGroup?: string
  status?: number
}

export interface JobLogQueryParams {
  jobId?: number
  status?: number
  startTime?: string
  endTime?: string
}
