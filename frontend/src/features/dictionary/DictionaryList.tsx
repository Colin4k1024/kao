import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Plus } from 'lucide-react'

export default function DictionaryList() {
  return (
    <div className="p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">字典管理</h1>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          新增字典
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>字典列表</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-muted-foreground">
            字典管理功能开发中...
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
