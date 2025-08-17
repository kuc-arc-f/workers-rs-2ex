import dayjs from "dayjs";

const apiPlan = {
  /**
  * 
  * @param
  *
  * @return
  */ 
  create: async function(item: any, path: string): Promise<any>
  {
    try {
      item.user_id = 1;
      if(item.p_date){
        const now = dayjs(item.p_date);
        item.p_date = now.format('YYYY-MM-DD HH:mm:ss');
      }
      console.log(item);
      const res = await fetch("/api/plan/create", {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},      
        body: JSON.stringify(item)
      });
      if (res.status !== 200) {
        throw new Error(await res.text());
      }
      const json = await res.json()
//console.log(json);   
      return json;
    } catch (e) {
      console.error(e);
      throw new Error('Error , create');
    }
  }, 
  /**
  * 
  * @param
  *
  * @return
  */ 
  update: async function(item: any, path: string): Promise<any>
  {
    try {
      item.userId = 1;
      const target = {
        path: "/plan/update",
        data: item,
      }
      //const body: any = JSON.stringify(item);	
      const res = await fetch("/api/plan/update", {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},      
        body: JSON.stringify(item)
      });
      if (res.status !== 200) {
        throw new Error(await res.text());
      }
      const json = await res.json()
//console.log(json);   
      return json;
    } catch (e) {
      console.error(e);
      throw new Error('Error , update');
    }
  }, 
  /**
  * 
  * @param
  *
  * @return
  */ 
  delete: async function(item: any, path: string): Promise<any>
  {
    try {
      item.userId = 1;
      const target = {
        path: "/plan/delete",
        data: item,
      }
      //const body: any = JSON.stringify(item);	
      const res = await fetch("/api/plan/delete", {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},      
        body: JSON.stringify(item)
      });
      if (res.status !== 200) {
        throw new Error(await res.text());
      }
      const json = await res.json()
//console.log(json);   
      return json;
    } catch (e) {
      console.error(e);
      throw new Error('Error , delete');
    }
  },
  /**
  *
  * @param
  *
  * @return
  */
  list :async function (target_date: any): Promise<any>
  {
    try{
      const now = dayjs(target_date);
      const startOfMonth = now.startOf('month');
      const startOfNextMonth = now.add(1, 'month').startOf('month');
      console.log(startOfNextMonth.format('YYYY-MM-DD'));
      // 1日前の日付を取得
      const yesterday = startOfNextMonth.subtract(1, 'day');
      console.log(yesterday.format('YYYY-MM-DD'));
      const postItem = {
        "start": startOfMonth.format('YYYY-MM-DD HH:mm:ss'),
        "end": startOfNextMonth.format('YYYY-MM-DD HH:mm:ss'),
        "user_id": 1
      }
      console.log(postItem);
      const target = {
        path: "/plan/get_list",
        data: postItem,
      }
      const res = await fetch("/api/plan/list_range", {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},      
        body: JSON.stringify(postItem)
      });
      if (res.status !== 200) {
        throw new Error(await res.text());
      }
      const json = await res.json()
      const out = this.getPlanData(json.data);   
      console.log(json);   
      return out;
    } catch (e) {
      console.error(e);
      throw new Error("Error, getList");
    } 
  }  ,

  getPlanData: function(items){
    let ret = []
    items.forEach((item) => {
      let row = item;
      row.date = item.p_date;
      //console.log(row)
      ret.push(item)
    });
    return ret;
  },
  
}
export default apiPlan;
