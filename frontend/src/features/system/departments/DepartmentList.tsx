import { useQuery } from '@tanstack/react-query'
import { deptApi } from '@/services/api/systemService'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Plus } from 'lucide-react'
import { Button } from '@/components/ui/button'

export default function DepartmentList() {
  const { data, isLoading } = useQuery({
    queryKey: ['departments'],
    queryFn: () => deptApi.list({}),
  })

  return (
    <div className="p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">部门管理</h1>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          新增部门
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>部门列表</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="text-center py-8">加载中...</div>
          ) : (
            <div className="space-y-2">
              {data?.data?.data?.map((dept) => (
                <div
                  key={dept.id}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-accent/50"
                >
                  <div>
                    <div className="font-medium">{dept.name}</div>
                    <div className="text-sm text-muted-foreground">
                      负责人: {dept.leader || '-'}
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    <span className={`px-2 py-1 rounded text-xs ${
                      dept.status === 1 ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                    }`}>
                      {dept.status === 1 ? '正常' : '禁用'}
                    </span>
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
