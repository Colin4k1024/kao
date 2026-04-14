# Phase 03 Plan 08: Frontend TypeScript Fix

**Phase:** 03-production-readiness
**Plan:** 08
**Date:** 2026-03-28
**Status:** PLANNING

---

## Executive Summary

**Problem:** Frontend has 157 TypeScript errors blocking `npm run build`
**Root Cause:** Code was written for UmiJS framework but project uses Vite + React Router
**Fix Strategy:** Replace UmiJS imports with Vite-compatible alternatives

---

## Error Categories

### Category 1: @umijs/max Imports (25 errors)
Files still importing from UmiJS (not available in Vite):
- `src/services/ant-design-pro/api.ts` - request
- `src/services/ant-design-pro/login.ts` - request
- `src/services/swagger/store.ts` - request
- `src/services/swagger/pet.ts` - request
- `src/services/swagger/user.ts` - request
- `src/components/RightContent/index.tsx` - SelectLang
- `src/pages/table-list/components/CreateForm.tsx` - FormattedMessage, useIntl, useRequest
- `src/pages/table-list/components/UpdateForm.tsx` - FormattedMessage, useIntl, useRequest
- `src/pages/table-list/index.tsx` - FormattedMessage, useIntl, useRequest
- `src/pages/user/login/index.tsx` - Helmet, SelectLang

**Fix:** Replace imports from `@/lib/umi-max-stub` or direct implementations

### Category 2: API Response Type Mismatches (15 errors)
Code accesses `.data` property on types that don't have it:
- `src/features/auth/LoginForm.tsx` - LoginResult.data
- `src/features/system/departments/DepartmentList.tsx` - Department[].data
- `src/features/system/menus/MenuList.tsx` - Menu[].data
- `src/features/system/posts/PostList.tsx` - PageResult<Post>.data
- `src/features/system/roles/RoleList.tsx` - PageResult<Role>.data
- `src/features/system/users/UserList.tsx` - PageResult<User>.data

**Fix:** Remove `.data` accessor or fix type definitions

### Category 3: CardInterface Issues (3 errors)
- `src/components/common/PageLayout.tsx` - Header/Content/Footer don't exist on CardInterface

**Fix:** Use Card.Header, Card.Content, Card.Footer or standard divs

### Category 4: ChartSectionProps Type Conflict (1 error)
- `src/components/dashboard/ChartSection.tsx` - ChartSectionProps.type conflicts with CardProps.type

**Fix:** Remove `type` from ChartSectionProps or change property name

### Category 5: ImportMeta.env Issues (2 errors)
- `src/lib/api.ts` - import.meta.env
- `src/lib/constants.ts` - import.meta.env

**Fix:** Add Vite type definitions or use `import.meta.env.VITE_*`

### Category 6: Unused Imports/Variables (40+ errors)
Multiple files have unused imports that TypeScript reports as errors

**Fix:** Remove unused imports

### Category 7: Implicit Any Types (20+ errors)
Various places have implicitly typed parameters

**Fix:** Add explicit type annotations

### Category 8: Duplicate Identifiers (3 errors)
- `src/pages/job/index.tsx` - duplicate `Modal`

**Fix:** Remove duplicate import

### Category 9: Missing Exports (10 errors)
- `RefreshOutlined` doesn't exist in @ant-design/icons (use `ReloadOutlined`)
- `Text`, `TextArea` missing from antd
- `ColumnsType` not found in type/index.tsx

### Category 10: Type Assertion Issues (5 errors)
- Column `fixed` property type mismatch (string vs FixedType)
- requestErrorConfig function signature mismatch

### Category 11: Test File Issues (15 errors)
- Missing @types/jest
- Cannot find module '@testing-library/react'
- describe/it/expect not defined

---

## Execution Order

1. **Fix @umijs/max imports** - 10 files
2. **Fix API response types** - 6 files
3. **Fix CardInterface** - PageLayout.tsx
4. **Fix ChartSectionProps** - ChartSection.tsx
5. **Fix ImportMeta.env** - api.ts, constants.ts
6. **Remove duplicate Modal** - job/index.tsx
7. **Fix antd exports** - RefreshOutlined → ReloadOutlined, add Text/TextArea
8. **Fix Column fixed type** - Cast to FixedType
9. **Fix requestErrorConfig** - function signature
10. **Remove unused imports** - across all files
11. **Add explicit types** - implicit any fixes
12. **Fix test file issues** - add @types/jest or skip tests

---

## Success Criteria

- [ ] `npx tsc --noEmit` passes with 0 errors
- [ ] `npm run build` succeeds
- [ ] Frontend dev server starts without errors

---

**Generated:** 2026-03-28
**Phase:** 03-production-readiness
**Plan:** 08 (Frontend TypeScript Fix)
