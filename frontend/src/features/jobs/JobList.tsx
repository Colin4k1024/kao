import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Plus } from 'lucide-react'

export default function JobList() {
  return (
    <div className="p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">定时任务</h1>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          新增任务
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>任务列表</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-muted-foreground">
            定时任务功能开发中...
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
