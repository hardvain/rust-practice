struct Sample {
  key: i32,
}
impl Sample {
  pub fn owned(self) {}
  pub fn shared_ref(&self) {}
  pub fn mutable_ref(&self) {}
}
fn main() {
  {
    {
      let sample = Sample { key: 1 };
      sample.owned();
      // cannot use below calls because value has been moved
      // sample.owned();
      // sample.shared_ref();
      // sample.mutable_ref();
    }
    {
      let sample = Sample { key: 1 };
      sample.shared_ref();
      sample.shared_ref();
      sample.mutable_ref();
      sample.mutable_ref();
      sample.owned();
      // sample.owned();
    }
    {
      let sample = Sample { key: 1 };
      sample.shared_ref();
      sample.owned();
      // cannot call below because value has been moved
      // sample.mutable_ref();
    }
    {
      let sample = Sample { key: 1 };
      sample.mutable_ref();
      sample.mutable_ref();
      sample.shared_ref();
      sample.shared_ref();
      sample.owned();
    }
  }

  {
    {
      let mut sample = Sample { key: 1 };
      sample.owned();
      // cannot use below calls because value has been moved
      // sample.owned();
      // sample.shared_ref();
      // sample.mutable_ref();
    }
    {
      let mut sample = Sample { key: 1 };
      sample.shared_ref();
      sample.shared_ref();
      sample.mutable_ref();
      sample.mutable_ref();
      sample.owned();
      // sample.owned();
    }
    {
      let mut sample = Sample { key: 1 };
      sample.shared_ref();
      sample.owned();
      // cannot call below because value has been moved
      // sample.mutable_ref();
    }
    {
      let mut sample = Sample { key: 1 };
      sample.mutable_ref();
      sample.mutable_ref();
      sample.shared_ref();
      sample.shared_ref();
      sample.owned();
    }
  }
}
