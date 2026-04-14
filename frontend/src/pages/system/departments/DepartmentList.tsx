import React, { useEffect, useState } from 'react';
import { Tree, Button, Space, Card } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import request from '@/lib/api';

interface Department {
  id: string;
  parent_id?: string;
  department_name: string;
  leader?: string;
  phone?: string;
  email?: string;
  status: number;
  children?: Department[];
}

const DepartmentList: React.FC = () => {
  const [departments, setDepartments] = useState<Department[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchDepartments = async () => {
    setLoading(true);
    try {
      const response = await request.get<{ data: Department[] }>('/api/v1/departments');
      setDepartments(response.data || []);
    } catch (error) {
      console.error('获取部门列表失败', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDepartments();
  }, []);

  const handleAdd = () => {
    console.log('新增部门');
  };

  const handleEdit = (department: Department) => {
    console.log('编辑部门', department);
  };

  const handleDelete = (department: Department) => {
    console.log('删除部门', department);
  };

  const renderTreeNodes = (data: Department[]): any[] => {
    return data.map((item) => ({
      key: item.id,
      title: (
        <Space>
          <span>{item.department_name}</span>
          {item.leader && <span style={{ color: '#999' }}>- {item.leader}</span>}
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={(e) => {
              e.stopPropagation();
              handleEdit(item);
            }}
          />
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={(e) => {
              e.stopPropagation();
              handleDelete(item);
            }}
          />
        </Space>
      ),
      children: item.children ? renderTreeNodes(item.children) : undefined,
    }));
  };

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增部门
          </Button>
          <Button onClick={fetchDepartments}>刷新</Button>
        </Space>
      </div>
      <Card>
        <Tree
          showLine
          defaultExpandAll
          treeData={loading ? [] : renderTreeNodes(departments)}
        />
      </Card>
    </div>
  );
};

export default DepartmentList;
