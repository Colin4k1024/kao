import { useQuery } from '@tanstack/react-query'
import { roleApi } from '@/services/api/systemService'
import type { Role } from '@/types/user'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Plus, Edit, Trash2, Settings } from 'lucide-react'

export default function RoleList() {
  const { data, isLoading } = useQuery({
    queryKey: ['roles'],
    queryFn: () => roleApi.list({ page: 1, pageSize: 100 }),
  })

  return (
    <div className="p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">角色管理</h1>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          新增角色
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>角色列表</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="text-center py-8">加载中...</div>
          ) : (
            <div className="space-y-2">
              {data?.list?.map((role: Role) => (
                <div
                  key={role.id}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-accent/50"
                >
                  <div>
                    <div className="font-medium">{role.name}</div>
                    <div className="text-sm text-muted-foreground">
                      标识: {role.code} | {role.description || '无描述'}
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    <span className={`px-2 py-1 rounded text-xs ${
                      role.status === 1 ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                    }`}>
                      {role.status === 1 ? '正常' : '禁用'}
                    </span>
                    <div className="flex items-center gap-2">
                      <Button variant="ghost" size="icon" title="分配权限">
                        <Settings className="h-4 w-4" />
                      </Button>
                      <Button variant="ghost" size="icon">
                        <Edit className="h-4 w-4" />
                      </Button>
                      <Button variant="ghost" size="icon">
                        <Trash2 className="h-4 w-4 text-red-500" />
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
