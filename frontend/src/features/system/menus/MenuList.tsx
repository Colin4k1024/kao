import { useQuery } from '@tanstack/react-query'
import { menuApi } from '@/services/api/systemService'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Plus, Edit, TreePine } from 'lucide-react'

export default function MenuList() {
  const { data, isLoading } = useQuery({
    queryKey: ['menus'],
    queryFn: () => menuApi.list({}),
  })

  const renderMenuTree = (menus: any[], level = 0) => {
    return menus?.map((menu) => (
      <div key={menu.id} style={{ marginLeft: level * 20 }}>
        <div className="flex items-center justify-between p-3 border rounded-lg mb-2 hover:bg-accent/50">
          <div className="flex items-center gap-3">
            <TreePine className="h-4 w-4 text-muted-foreground" />
            <div>
              <div className="font-medium">{menu.name}</div>
              <div className="text-sm text-muted-foreground">
                路径: {menu.path} | 类型: {menu.type === 'M' ? '目录' : menu.type === 'C' ? '菜单' : '按钮'}
              </div>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="icon">
              <Edit className="h-4 w-4" />
            </Button>
          </div>
        </div>
        {menu.children && renderMenuTree(menu.children, level + 1)}
      </div>
    ))
  }

  return (
    <div className="p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">菜单管理</h1>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          新增菜单
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>菜单树</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="text-center py-8">加载中...</div>
          ) : (
            <div>
              {renderMenuTree(Array.isArray(data) ? data : [])}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
