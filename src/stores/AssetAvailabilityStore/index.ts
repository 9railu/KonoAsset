import { CloudAvailabilityStatus, commands } from '@/lib/bindings'
import { create } from 'zustand'

type Props = {
  statuses: Partial<Record<string, CloudAvailabilityStatus>>
  refreshStatuses: (ids: string[]) => Promise<void>
}

// アセットカード一覧が表示されているタイミングで一括取得し、カード個別には
// バックエンドを呼ばせない（N+1呼び出しを避けるため）。
export const useAssetAvailabilityStore = create<Props>((set) => ({
  statuses: {},
  refreshStatuses: async (ids: string[]) => {
    if (ids.length === 0) {
      return
    }

    const result = await commands.getAssetAvailabilityStatuses(ids)

    if (result.status === 'error') {
      console.error(result.error)
      return
    }

    set((prev) => ({
      statuses: { ...prev.statuses, ...result.data },
    }))
  },
}))
