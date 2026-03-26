import { useQuery } from '@tanstack/react-query'
import { postApi } from '@/services/api/systemService'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Plus, Edit, Trash2 } from 'lucide-react'

export default function PostList() {
  const { data, isLoading } = useQuery({
    queryKey: ['posts'],
    queryFn: () => postApi.list({ page: 1, pageSize: 100 }),
  })

  return (
    <div className="p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">岗位管理</h1>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          新增岗位
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>岗位列表</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="text-center py-8">加载中...</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b">
                    <th className="text-left p-3">ID</th>
                    <th className="text-left p-3">岗位名称</th>
                    <th className="text-left p-3">岗位编码</th>
                    <th className="text-left p-3">排序</th>
                    <th className="text-left p-3">状态</th>
                    <th className="text-left p-3">备注</th>
                    <th className="text-left p-3">操作</th>
                  </tr>
                </thead>
                <tbody>
                  {data?.data?.data?.list?.map((post) => (
                    <tr key={post.id} className="border-b hover:bg-accent/50">
                      <td className="p-3">{post.id}</td>
                      <td className="p-3">{post.name}</td>
                      <td className="p-3">{post.code}</td>
                      <td className="p-3">{post.sort}</td>
                      <td className="p-3">
                        <span className={`px-2 py-1 rounded text-xs ${
                          post.status === 1 ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                        }`}>
                          {post.status === 1 ? '正常' : '禁用'}
                        </span>
                      </td>
                      <td className="p-3">{post.remark || '-'}</td>
                      <td className="p-3">
                        <div className="flex items-center gap-2">
                          <Button variant="ghost" size="icon">
                            <Edit className="h-4 w-4" />
                          </Button>
                          <Button variant="ghost" size="icon">
                            <Trash2 className="h-4 w-4 text-red-500" />
                          </Button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
