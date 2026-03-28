import { QuestionCircleOutlined } from '@ant-design/icons';
import { SelectLang } from '@/lib/umi-max-stub';

export type SiderTheme = 'light' | 'dark';
export { SelectLang };

export const Question: React.FC = () => {
  return (
    <a
      href="https://pro.ant.design/docs/getting-started"
      target="_blank"
      rel="noreferrer"
      style={{
        display: 'inline-flex',
        padding: '4px',
        fontSize: '18px',
        color: 'inherit',
      }}
    >
      <QuestionCircleOutlined />
    </a>
  );
};
