use gtk::prelude::*;
use gtk::glib;
use super::sqlentity::EntityFromSql;
use super::*;

pub struct SqlListStoreWorker<E>
where
    E: EntityFromSql + IsA<glib::Object>,
{
    object_cache: Vec<E>,
    sorter: Option<gtk::Sorter>,
    max_accessed_position: u32,
    fetch_margin: u32,
    fetch_amount: u32,
    sort_by:String,
    table_name: String,
}

impl<E> SqlListStoreWorker<E>
where
    E: EntityFromSql + IsA<glib::Object>,
{
    const DEFAULT_FETCH_MARGIN: u32 = 200;
    const DEFAULT_FETCH_AMOUNT: u32 = 200;

    pub fn n_items(&self) -> u32 {
        TryInto::<u32>::try_into(self.object_cache.len()).unwrap()
    }

    pub fn clear_items(&mut self) -> u32 {
        let n_items = self.n_items();
        self.object_cache.clear();
        self.max_accessed_position = 0;
        n_items
    }

    pub fn fetch_entities(&self,
        offset: u32,
        row_count: u32,
    ) -> Result<Vec<E>, Box<dyn std::error::Error>> {
        let mut query = std::string::String::from("");

        query += &format!("SELECT * FROM {} {} LIMIT {}, {};", self.table_name, self.sort_by, offset.to_string(), row_count.to_string());

        println!("{:}", query);

        let mut vec = vec![];
        for entity in CONNECTION
            .lock()?
            .prepare(&query)?
            .query_map([], |row| E::try_new_from_row(row))?
        {
            if let Ok(entity) = entity {
                vec.push(entity)
            }
        }

        println!("{:} items loaded.", vec.len());
        Ok(vec)
    }

    pub fn fetch_more_if_necessary(&mut self) -> u32 {
        let n_items = self.n_items();
        if self.max_accessed_position + self.fetch_margin >= n_items {
            let n_items_to_fetch = u32::max(
                self.fetch_amount,
                self.max_accessed_position + self.fetch_margin - n_items + 1,
            );

            // FIXME: fetching takes too long when entities are sorted by datetime column
            match self.fetch_entities(self.n_items(), n_items_to_fetch)
            {
                Ok(mut entities) => {
                    let added = entities.len();
                    self.object_cache.append(&mut entities);
                    added.try_into().unwrap()
                }
                Err(e) => {
                    dbg!(e);
                    0
                }
            }
        } else {
            0
        }
    }

    pub fn item(&mut self, position: u32) -> Option<glib::Object> {
        //dbg!(position);
        if position > self.max_accessed_position {
            self.max_accessed_position = position;
        }

        if let Some(item) = self.object_cache.get(position as usize) {
            Some(item.clone().upcast())
        } else {
            None
        }
    }

    pub fn update_sort_by(&mut self) {
        self.sort_by = "".to_string();

        if let Some(sorter) = self.sorter.clone() {
            if let Ok(sorter) = sorter.downcast::<gtk::ColumnViewSorter>() {
                self.sort_by = "ORDER BY".to_string();
                let n_sort_columns = sorter.n_sort_columns();
                for position in 0..n_sort_columns {
                    let (column, order) = sorter.nth_sort_column(position);
                    if let Some(column) = column {
                        if let Some(column_id) = column.id() {
                            let order = match order {
                                gtk::SortType::Ascending => SortOrder::Ascending,
                                gtk::SortType::Descending => SortOrder::Descending,
                                _ => SortOrder::Ascending,
                            };
                            self.sort_by += &format!(" {} {}", column_id.to_string(), order.to_str());
                            if position < n_sort_columns - 1 {
                                self.sort_by += &",";
                            }
                        }
                    }
                }
            }
        }
        dbg!(&self.sort_by);
    }

    pub fn set_sorter(&mut self, sorter: Option<gtk::Sorter>) {
        self.sorter = sorter;
    }

    pub fn set_table_name(&mut self, table_name: String) {
        self.table_name = table_name;
    }
}

impl<E> Default for SqlListStoreWorker<E>
where
    E: EntityFromSql + IsA<glib::Object>,
{
    fn default() -> Self {
        Self {
            object_cache: Default::default(),
            sorter: Default::default(),
            sort_by: Default::default(),
            max_accessed_position: Default::default(),
            fetch_margin: Self::DEFAULT_FETCH_MARGIN,
            fetch_amount: Self::DEFAULT_FETCH_AMOUNT,
            table_name: "".to_string(),
        }
    }
}
