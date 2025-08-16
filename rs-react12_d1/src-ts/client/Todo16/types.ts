// 投稿アイテムのデータ構造を定義
export interface Item {
  id: number; // バックエンドで自動採番される想定
  title: string;
  content: string;
  contentType: string;
  publicType: 'public' | 'private' | 'draft'; // ラジオボタンの値の例
  foodOrange: boolean;
  foodApple: boolean;
  foodBanana: boolean;
  foodMelon: boolean;
  foodGrape: boolean;
  categoryFood: boolean;
  categoryDrink: boolean;
  categoryGadget: boolean;
  categorySport: boolean;
  categoryGovernment: boolean;
  categoryInternet: boolean;
  categorySmartphone: boolean;
  countryJp: string;
  countryEn: string;
  prefectureJp: string;
  prefectureEn: string;
  postNoJp: string;
  postNoEn: string;
  address1Jp: string;
  address1En: string;
  address2Jp: string;
  address2En: string;
  addressOtherJp: string;
  addressOtherEn: string;
  pubDate1: string; // "YYYY-MM-DD"形式
  pubDate2: string;
  pubDate3: string;
  pubDate4: string;
  pubDate5: string;
  pubDate6: string;
  qty1: string;
  qty2: string;
  qty3: string;
  qty4: string;
  qty5: string;
  qty6: string;
}

// Itemからidを除いた型 (新規作成時に使用)
export type ItemData = Omit<Item, 'id'>;